import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [synthState, setSynthState] = useState<string>("");
  const [frequency, setFrequency] = useState<number>(440);

  async function playNote() {
    // This will call our Rust backend
    setSynthState(await invoke("play_note", { frequency: frequency }));
  }

  async function stopNote() {
    setSynthState(await invoke("stop_note"));
  }

  return (
    <div className="container">
      <h1>SynthMob - Mobile Synthesizer</h1>
      
      <div className="row">
        <div>
          <input
            id="frequency-input"
            type="number"
            value={frequency}
            onChange={(e) => setFrequency(parseInt(e.currentTarget.value))}
            placeholder="Enter frequency..."
          />
          <label htmlFor="frequency-input">Frequency (Hz)</label>
        </div>
        
        <button type="button" onMouseDown={playNote} onMouseUp={stopNote} onTouchStart={playNote} onTouchEnd={stopNote}>
          Play Note
        </button>
      </div>

      <p>{synthState}</p>
    </div>
  );
}

export default App;
