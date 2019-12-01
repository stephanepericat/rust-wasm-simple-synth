mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{AudioContext, OscillatorType};

/// Converts a midi note to frequency
///
/// A midi note is an integer, generally in the range of 21 to 108
pub fn midi_to_freq(note: u8) -> f32 {
    27.5 * 2f32.powf((note as f32 - 21.0) / 12.0)
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f32(a: f32);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(a: f64);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u8(a: u8);
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct SimpleSynth {
    ctx: AudioContext,
    gain: web_sys::GainNode,
    osc: web_sys::OscillatorNode
}

impl Drop for SimpleSynth {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

#[wasm_bindgen]
impl SimpleSynth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SimpleSynth, JsValue> {
        utils::set_panic_hook();

        let ctx = web_sys::AudioContext::new()?;
        let osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;
        let time = ctx.current_time();

        osc.set_type(OscillatorType::Sine);
        osc.frequency().set_value_at_time(440.0, time).ok();
        gain.gain().set_value_at_time(0.1, time).ok();

        osc.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&ctx.destination())?;

        osc.start()?;

        Ok(SimpleSynth {
            ctx,
            gain,
            osc
        })
    }

    fn get_ctx_time(&mut self) -> f64 {
        self.ctx.current_time()
    }

    pub fn set_note(&mut self, note: u8) -> f32 {
        let time = self.get_ctx_time();
        let freq = midi_to_freq(note);
        self.osc.frequency().set_value_at_time(freq, time).ok();
        freq
    }

    pub fn set_volume(&mut self, volume: f32) {
        let time = self.get_ctx_time();
        self.gain.gain().set_value_at_time(volume, time).ok();
    }
}
