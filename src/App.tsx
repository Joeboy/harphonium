import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

function App() {
  const [synthState, setSynthState] = useState<string>("");
  const [frequency, setFrequency] = useState<number>(440);

  async function playNote() {
    try {
      // This will call our Rust backend
      const result = await invoke("play_note", { frequency: frequency });
      setSynthState(result as string);
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
  }

  async function stopNote() {
    try {
      const result = await invoke("stop_note");
      setSynthState(result as string);
    } catch (error) {
      setSynthState(`Error: ${error}`);
    }
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
