import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './SynthTab.css';

interface SynthTabProps {
  // Add props for synth parameters as needed
}

const SynthTab: React.FC<SynthTabProps> = () => {
  const [oscillatorType, setOscillatorType] = useState<'sine' | 'square' | 'sawtooth' | 'triangle'>('sine');
  const [attackTime, setAttackTime] = useState(0.1);
  const [decayTime, setDecayTime] = useState(0.2);
  const [sustainLevel, setSustainLevel] = useState(0.7);
  const [releaseTime, setReleaseTime] = useState(0.3);
  const [filterCutoff, setFilterCutoff] = useState(1000);
  const [filterResonance, setFilterResonance] = useState(0.5);
  const [masterVolume, setMasterVolume] = useState(70);

  // Load initial master volume on component mount
  useEffect(() => {
    const loadMasterVolume = async () => {
      try {
        const volume: number = await invoke('get_master_volume');
        setMasterVolume(Math.round(volume * 100)); // Convert from 0-1 to 0-100 for UI
      } catch (error) {
        console.error('Failed to get master volume:', error);
      }
    };
    loadMasterVolume();
  }, []);

  // Handle master volume changes
  const handleMasterVolumeChange = async (value: number) => {
    setMasterVolume(value);
    try {
      await invoke('set_master_volume', { volume: value / 100 }); // Convert from 0-100 to 0-1
    } catch (error) {
      console.error('Failed to set master volume:', error);
    }
  };

  return (
    <div className="synth-tab">
      
      <div className="synth-section">
        <h3>Master</h3>
        <div className="control-group">
          <label htmlFor="master-volume">Master Volume: {masterVolume}%</label>
          <input
            type="range"
            id="master-volume"
            min="0"
            max="100"
            step="1"
            value={masterVolume}
            onChange={(e) => handleMasterVolumeChange(parseInt(e.target.value))}
          />
        </div>
      </div>
      
      <div className="synth-section">
        <h3>Oscillator</h3>
        <div className="control-group">
          <label htmlFor="oscillator-type">Waveform:</label>
          <select
            id="oscillator-type"
            value={oscillatorType}
            onChange={(e) => setOscillatorType(e.target.value as any)}
          >
            <option value="sine">Sine</option>
            <option value="square">Square</option>
            <option value="sawtooth">Sawtooth</option>
            <option value="triangle">Triangle</option>
          </select>
        </div>
      </div>

      <div className="synth-section">
        <h3>ADSR Envelope</h3>
        <div className="control-group">
          <label htmlFor="attack">Attack: {attackTime.toFixed(2)}s</label>
          <input
            type="range"
            id="attack"
            min="0.01"
            max="2"
            step="0.01"
            value={attackTime}
            onChange={(e) => setAttackTime(parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label htmlFor="decay">Decay: {decayTime.toFixed(2)}s</label>
          <input
            type="range"
            id="decay"
            min="0.01"
            max="2"
            step="0.01"
            value={decayTime}
            onChange={(e) => setDecayTime(parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label htmlFor="sustain">Sustain: {(sustainLevel * 100).toFixed(0)}%</label>
          <input
            type="range"
            id="sustain"
            min="0"
            max="1"
            step="0.01"
            value={sustainLevel}
            onChange={(e) => setSustainLevel(parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label htmlFor="release">Release: {releaseTime.toFixed(2)}s</label>
          <input
            type="range"
            id="release"
            min="0.01"
            max="3"
            step="0.01"
            value={releaseTime}
            onChange={(e) => setReleaseTime(parseFloat(e.target.value))}
          />
        </div>
      </div>

      <div className="synth-section">
        <h3>Filter</h3>
        <div className="control-group">
          <label htmlFor="cutoff">Cutoff: {filterCutoff.toFixed(0)} Hz</label>
          <input
            type="range"
            id="cutoff"
            min="100"
            max="8000"
            step="10"
            value={filterCutoff}
            onChange={(e) => setFilterCutoff(parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label htmlFor="resonance">Resonance: {(filterResonance * 100).toFixed(0)}%</label>
          <input
            type="range"
            id="resonance"
            min="0"
            max="1"
            step="0.01"
            value={filterResonance}
            onChange={(e) => setFilterResonance(parseFloat(e.target.value))}
          />
        </div>
      </div>

      <div className="synth-section">
        <h3>Effects</h3>
        <div className="control-group">
          <label>
            <input type="checkbox" />
            Reverb
          </label>
        </div>
        <div className="control-group">
          <label>
            <input type="checkbox" />
            Delay
          </label>
        </div>
        <div className="control-group">
          <label>
            <input type="checkbox" />
            Chorus
          </label>
        </div>
      </div>
    </div>
  );
};

export default SynthTab;
