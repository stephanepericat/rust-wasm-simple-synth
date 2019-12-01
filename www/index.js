import * as wasm from "simple-synth";

const synth = new wasm.SimpleSynth();

const tick = () => {
  synth.tick();
  requestAnimationFrame(tick);
};

// tick();
