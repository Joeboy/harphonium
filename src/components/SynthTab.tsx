import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './SynthTab.css';

interface SynthTabProps {
  // Add props for synth parameters as needed
}

const SynthTab: React.FC<SynthTabProps> = () => {
  const [oscillatorType, setOscillatorType] = useState<
    'sine' | 'square' | 'sawtooth' | 'triangle'
  >('sine');
  const [attackTime, setAttackTime] = useState(0.1);
  const [decayTime, setDecayTime] = useState(0.2);
  const [sustainLevel, setSustainLevel] = useState(0.7);
  const [releaseTime, setReleaseTime] = useState(0.3);
  const [filterCutoff, setFilterCutoff] = useState(1000);
  const [filterResonance, setFilterResonance] = useState(0.5);
  const [masterVolume, setMasterVolume] = useState(70);

  // Load initial values on component mount
  useEffect(() => {
    const loadInitialValues = async () => {
      try {
        // Load master volume
        const volume: number = await invoke('get_master_volume');
        setMasterVolume(Math.round(volume * 100)); // Convert from 0-1 to 0-100 for UI

        // Load current waveform
        const waveform: string = await invoke('get_waveform');
        setOscillatorType(
          waveform as 'sine' | 'square' | 'sawtooth' | 'triangle'
        );

        // Load ADSR values
        const attack: number = await invoke('get_attack');
        setAttackTime(attack);

        const decay: number = await invoke('get_decay');
        setDecayTime(decay);

        const sustain: number = await invoke('get_sustain');
        setSustainLevel(sustain);

        const release: number = await invoke('get_release');
        setReleaseTime(release);
      } catch (error) {
        console.error('Failed to load initial values:', error);
      }
    };
    loadInitialValues();
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

  // Handle waveform changes
  const handleWaveformChange = async (
    waveform: 'sine' | 'square' | 'sawtooth' | 'triangle'
  ) => {
    setOscillatorType(waveform);
    try {
      await invoke('set_waveform', { waveform });
      console.log(`Waveform changed to: ${waveform}`);
    } catch (error) {
      console.error('Failed to set waveform:', error);
    }
  };

  // Handle ADSR attack changes
  const handleAttackChange = async (attack: number) => {
    setAttackTime(attack);
    try {
      await invoke('set_attack', { attack });
    } catch (error) {
      console.error('Failed to set attack:', error);
    }
  };

  // Handle ADSR decay changes
  const handleDecayChange = async (decay: number) => {
    setDecayTime(decay);
    try {
      await invoke('set_decay', { decay });
    } catch (error) {
      console.error('Failed to set decay:', error);
    }
  };

  // Handle ADSR sustain changes
  const handleSustainChange = async (sustain: number) => {
    setSustainLevel(sustain);
    try {
      await invoke('set_sustain', { sustain });
    } catch (error) {
      console.error('Failed to set sustain:', error);
    }
  };

  // Handle ADSR release changes
  const handleReleaseChange = async (release: number) => {
    setReleaseTime(release);
    try {
      await invoke('set_release', { release });
    } catch (error) {
      console.error('Failed to set release:', error);
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
            onChange={(e) =>
              handleWaveformChange(
                e.target.value as 'sine' | 'square' | 'sawtooth' | 'triangle'
              )
            }
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
            onChange={(e) => handleAttackChange(parseFloat(e.target.value))}
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
            onChange={(e) => handleDecayChange(parseFloat(e.target.value))}
          />
        </div>
        <div className="control-group">
          <label htmlFor="sustain">
            Sustain: {(sustainLevel * 100).toFixed(0)}%
          </label>
          <input
            type="range"
            id="sustain"
            min="0"
            max="1"
            step="0.01"
            value={sustainLevel}
            onChange={(e) => handleSustainChange(parseFloat(e.target.value))}
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
            onChange={(e) => handleReleaseChange(parseFloat(e.target.value))}
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
          <label htmlFor="resonance">
            Resonance: {(filterResonance * 100).toFixed(0)}%
          </label>
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
