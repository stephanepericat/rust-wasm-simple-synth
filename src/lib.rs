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
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub struct SimpleSynth {
    ctx: AudioContext,
    gain: web_sys::GainNode,
    osc: web_sys::OscillatorNode
}

#[wasm_bindgen]
impl SimpleSynth {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<SimpleSynth, JsValue> {
        utils::set_panic_hook();

        let ctx = web_sys::AudioContext::new()?;
        let osc = ctx.create_oscillator()?;
        let gain = ctx.create_gain()?;

        osc.set_type(OscillatorType::Sine);
        osc.frequency().set_value(440.0);
        gain.gain().set_value(0.1);

        osc.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&ctx.destination())?;

        osc.start()?;

        Ok(SimpleSynth {
            ctx,
            gain,
            osc
        })
    }
}
