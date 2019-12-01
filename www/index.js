import { SimpleSynth } from "simple-synth"

const synth = new SimpleSynth()

const keys = document.querySelectorAll(".keyboard li")

keys.forEach(key => {
  key.addEventListener("click", e => {
    const note = Number(e.target.dataset.midi) || 69
    const f = synth.set_note(note)
    console.log("FREQUENCY >>>", f)
  })
})

const volumeCtrl = document.getElementById("volume-ctrl")
volumeCtrl.addEventListener("change", e => {
  const v = Number(e.target.value) || 0.1
  synth.set_volume(v)
  console.log("VOLUME >>>", v)
})