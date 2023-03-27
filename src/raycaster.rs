use cgmath::{Vector3, InnerSpace};
use wasm_bindgen::prelude::*;
use rayon::prelude::*;

use futures_channel::oneshot;
use js_sys::{Promise, Uint8ClampedArray, WebAssembly};

use crate::{pool, scene::Scene, camera::Camera};

#[wasm_bindgen]
pub struct Raycaster {
    pub width: u32,
    pub height: u32,
}

#[wasm_bindgen]
impl Raycaster {
    #[wasm_bindgen(constructor)]
    pub fn new(width: u32, height: u32) -> Result<Raycaster, JsValue> {
        console_error_panic_hook::set_once();

        Ok(Raycaster {
            width,
            height
        })
    }

    pub fn render(
        self,
        concurrency: usize,
        pool: &pool::WorkerPool,
    ) -> Result<RenderResult, JsValue> {
        let width = self.width;
        let height = self.height;

        // Allocate the pixel data which our threads will be writing into.
        let pixels = (width * height) as usize;
        let mut rgb_data = vec![0; 4 * pixels];
        let base = rgb_data.as_ptr() as usize;
        let len = rgb_data.len();

        let ns = 100;
        let camera = Camera::new(
            Vector3::new(-5.5, 2.0, 10.0),
            Vector3::new(0.8, 0.0, -0.6).normalize(),
            width as f32,
            height as f32
        );

        let scene = Scene::new();

        // Configure a rayon thread pool which will pull web workers from
        // `pool`.
        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(concurrency)
            .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
            .build()
            .unwrap();

        // And now execute the render! The entire render happens on our worker
        // threads so we don't lock up the main thread, so we ship off a thread
        // which actually does the whole rayon business. When our returned
        // future is resolved we can pull out the final version of the image.
        let (tx, rx) = oneshot::channel();
        let _ = pool.run(move || {
            thread_pool.install(|| {
                rgb_data
                    .par_chunks_mut(4)
                    .enumerate()
                    .for_each(|(i, chunk)| {
                        let i = i as u32;
                        let x = (i % width) as f32;
                        let y = (i / width) as f32;

                        let mut tot = Vector3::new(0.0, 0.0, 0.0);

                        for _ in 0..ns {
                            let rx: f32 = 2. * rand::random::<f32>() - 1.;
                            let ry: f32 = 2. * rand::random::<f32>() - 1.;

                            let mut ray = camera.get_ray(x + rx, y + ry);
                            let col = ray.color(&scene);

                            tot.x += col.x;
                            tot.y += col.y;
                            tot.z += col.z;
                        }

                        // chunk[0] = (tot.x * 255.0 / (ns as f32)) as u8;
                        // chunk[1] = (tot.y * 255.0 / (ns as f32)) as u8;
                        // chunk[2] = (tot.z * 255.0 / (ns as f32)) as u8;

                        chunk[0] = (gamma_encode(tot.x / ns as f32) * 255.0) as u8;
                        chunk[1] = (gamma_encode(tot.y / ns as f32) * 255.0) as u8;
                        chunk[2] = (gamma_encode(tot.z / ns as f32) * 255.0) as u8;
                        chunk[3] = 255 as u8;
                    });
            });
            drop(tx.send(rgb_data));
        });

        let done = async move {
            match rx.await {
                Ok(_data) => Ok(image_data(base, len, width, height).into()),
                Err(_) => Err(JsValue::undefined()),
            }
        };

        Ok(RenderResult {
            promise: wasm_bindgen_futures::future_to_promise(done),
            base,
            len,
            height,
            width,
        })
    }
}

#[wasm_bindgen]
extern "C" {
    pub type ImageData;

    #[wasm_bindgen(constructor, catch)]
    fn new(data: &Uint8ClampedArray, width: f64, height: f64) -> Result<ImageData, JsValue>;
}

#[wasm_bindgen]
pub struct RenderResult {
    base: usize,
    len: usize,
    width: u32,
    height: u32,
    promise: Promise,
}

#[wasm_bindgen]
impl RenderResult {
    /// Returns the JS promise object which resolves when the render is complete
    pub fn promise(&self) -> Promise {
        self.promise.clone()
    }

    /// Return a progressive rendering of the image so far
    #[wasm_bindgen(js_name = imageSoFar)]
    pub fn image_so_far(&self) -> ImageData {
        image_data(self.base, self.len, self.width, self.height)
    }
}

fn image_data(base: usize, len: usize, width: u32, height: u32) -> ImageData {
    // Use the raw access available through `memory.buffer`, but be sure to
    // use `slice` instead of `subarray` to create a copy that isn't backed
    // by `SharedArrayBuffer`. Currently `ImageData` rejects a view of
    // `Uint8ClampedArray` that's backed by a shared buffer.
    //
    // FIXME: that this may or may not be UB based on Rust's rules. For example
    // threads may be doing unsynchronized writes to pixel data as we read it
    // off here. In the context of wasm this may or may not be UB, we're
    // unclear! In any case for now it seems to work and produces a nifty
    // progressive rendering. A more production-ready application may prefer to
    // instead use some form of signaling here to request an update from the
    // workers instead of synchronously acquiring an update, and that way we
    // could ensure that even on the Rust side of things it's not UB.
    let mem = wasm_bindgen::memory().unchecked_into::<WebAssembly::Memory>();
    let mem = Uint8ClampedArray::new(&mem.buffer()).slice(base as u32, (base + len) as u32);
    ImageData::new(&mem, width as f64, height as f64).unwrap()
}

fn gamma_encode(linear: f32) -> f32 {
    linear.powf(1.0 / 2.2)
}
