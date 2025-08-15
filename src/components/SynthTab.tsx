import React, { useState, useEffect, useMemo } from 'react';
import throttle from 'lodash.throttle';

const SLIDER_THROTTLE_MS = 100;
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

  // Throttled master volume handler (20Hz)
  const throttledSetMasterVolume = useMemo(
    () =>
      throttle(async (value: number) => {
        try {
          await invoke('set_master_volume', { volume: value / 100 });
        } catch (error) {
          console.error('Failed to set master volume:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  const handleMasterVolumeChange = (value: number) => {
    setMasterVolume(value);
    throttledSetMasterVolume(value);
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

  // Throttled ADSR handlers (20Hz)
  const throttledSetAttack = useMemo(
    () =>
      throttle(async (attack: number) => {
        try {
          await invoke('set_attack', { attack });
        } catch (error) {
          console.error('Failed to set attack:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  const throttledSetDecay = useMemo(
    () =>
      throttle(async (decay: number) => {
        try {
          await invoke('set_decay', { decay });
        } catch (error) {
          console.error('Failed to set decay:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  const throttledSetSustain = useMemo(
    () =>
      throttle(async (sustain: number) => {
        try {
          await invoke('set_sustain', { sustain });
        } catch (error) {
          console.error('Failed to set sustain:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  const throttledSetRelease = useMemo(
    () =>
      throttle(async (release: number) => {
        try {
          await invoke('set_release', { release });
        } catch (error) {
          console.error('Failed to set release:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  // Handle ADSR changes (update state immediately, throttle backend update)
  const handleAttackChange = (attack: number) => {
    setAttackTime(attack);
    throttledSetAttack(attack);
  };
  const handleDecayChange = (decay: number) => {
    setDecayTime(decay);
    throttledSetDecay(decay);
  };
  const handleSustainChange = (sustain: number) => {
    setSustainLevel(sustain);
    throttledSetSustain(sustain);
  };
  const handleReleaseChange = (release: number) => {
    setReleaseTime(release);
    throttledSetRelease(release);
  };

  return (
    <div className="synth-tab">
      <div className="synth-section">
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
        <h3>ADSR</h3>
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
        <h3>Filter (TODO)</h3>
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
    </div>
  );
};

export default SynthTab;
