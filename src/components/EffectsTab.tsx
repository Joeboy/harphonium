import React, { useState, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './EffectsTab.css';
import throttle from 'lodash.throttle';

interface EffectsTabProps {
  // Add props for effects parameters as needed
}

const EffectsTab: React.FC<EffectsTabProps> = () => {
  // Reverb controls
  const [reverbEnabled, setReverbEnabled] = useState(false);
  const [reverbRoomSize, setReverbRoomSize] = useState(0.5);
  const [reverbDamping, setReverbDamping] = useState(0.3);
  const [reverbWetLevel, setReverbWetLevel] = useState(0.3);
  const [reverbDryLevel, setReverbDryLevel] = useState(0.7);

  // Delay controls
  // ...existing code...
  const [delayTime, setDelayTime] = useState(0.25);

  // ...existing code...
  const [delayWetLevel, setDelayWetLevel] = useState(0.3);


  // Throttled delay time and mix handlers (20Hz)
  const SLIDER_THROTTLE_MS = 100;
  const throttledSetDelayTime = useMemo(
    () =>
      throttle(async (value: number) => {
        try {
          await invoke('set_delay_time', { delayTime: value });
        } catch (error) {
          console.error('Failed to set delay time:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );
  const throttledSetDelayMix = useMemo(
    () =>
      throttle(async (value: number) => {
        try {
          await invoke('set_delay_mix', { delayMix: value });
        } catch (error) {
          console.error('Failed to set delay mix:', error);
        }
      }, SLIDER_THROTTLE_MS),
    []
  );

  const handleDelayTimeChange = (value: number) => {
    setDelayTime(value);
    throttledSetDelayTime(value);
  };

  const handleDelayMixChange = (value: number) => {
    setDelayWetLevel(value);
    throttledSetDelayMix(value);
  };

  // Chorus controls
  const [chorusEnabled, setChorusEnabled] = useState(false);
  const [chorusRate, setChorusRate] = useState(1.5);
  const [chorusDepth, setChorusDepth] = useState(0.3);
  const [chorusMix, setChorusMix] = useState(0.5);

  // Distortion controls
  const [distortionEnabled, setDistortionEnabled] = useState(false);
  const [distortionDrive, setDistortionDrive] = useState(0.3);
  const [distortionTone, setDistortionTone] = useState(0.5);
  const [distortionLevel, setDistortionLevel] = useState(0.7);

  // Compressor controls
  const [compressorEnabled, setCompressorEnabled] = useState(false);
  const [compressorThreshold, setCompressorThreshold] = useState(-20);
  const [compressorRatio, setCompressorRatio] = useState(4);
  const [compressorAttack, setCompressorAttack] = useState(0.003);
  const [compressorRelease, setCompressorRelease] = useState(0.1);

  return (
    <div className="effects-tab">
      <h2>Audio Effects</h2>

      {/* Reverb Section */}
      <div className="effect-section">
        <div className="effect-header">
          <h3>Reverb</h3>
          <label className="effect-toggle">
            <input
              type="checkbox"
              checked={reverbEnabled}
              onChange={(e) => setReverbEnabled(e.target.checked)}
            />
            <span className="toggle-text">{reverbEnabled ? 'ON' : 'OFF'}</span>
          </label>
        </div>
        {reverbEnabled && (
          <div className="effect-controls">
            <div className="control-group">
              <label htmlFor="reverb-room-size">
                Room Size: {(reverbRoomSize * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="reverb-room-size"
                min="0"
                max="1"
                step="0.01"
                value={reverbRoomSize}
                onChange={(e) => setReverbRoomSize(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="reverb-damping">
                Damping: {(reverbDamping * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="reverb-damping"
                min="0"
                max="1"
                step="0.01"
                value={reverbDamping}
                onChange={(e) => setReverbDamping(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="reverb-wet">
                Wet: {(reverbWetLevel * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="reverb-wet"
                min="0"
                max="1"
                step="0.01"
                value={reverbWetLevel}
                onChange={(e) => setReverbWetLevel(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="reverb-dry">
                Dry: {(reverbDryLevel * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="reverb-dry"
                min="0"
                max="1"
                step="0.01"
                value={reverbDryLevel}
                onChange={(e) => setReverbDryLevel(parseFloat(e.target.value))}
              />
            </div>
          </div>
        )}
      </div>

      {/* Delay Section */}
      <div className="effect-section">
        <div className="effect-header">
          <h3>Delay</h3>
        </div>
        <div className="effect-controls">
          <div className="control-group">
            <label htmlFor="delay-time">
              Time: {(delayTime * 1000).toFixed(0)}ms
            </label>
            <input
              type="range"
              id="delay-time"
              min="0.01"
              max="2"
              step="0.01"
              value={delayTime}
              onChange={(e) =>
                handleDelayTimeChange(parseFloat(e.target.value))
              }
            />
          </div>
          <div className="control-group">
            <label htmlFor="delay-wet">
              Mix: {(delayWetLevel * 100).toFixed(0)}%
            </label>
            <input
              type="range"
              id="delay-wet"
              min="0"
              max="1"
              step="0.01"
              value={delayWetLevel}
              onChange={(e) => handleDelayMixChange(parseFloat(e.target.value))}
            />
          </div>
        </div>
      </div>

      {/* Chorus Section */}
      <div className="effect-section">
        <div className="effect-header">
          <h3>Chorus</h3>
          <label className="effect-toggle">
            <input
              type="checkbox"
              checked={chorusEnabled}
              onChange={(e) => setChorusEnabled(e.target.checked)}
            />
            <span className="toggle-text">{chorusEnabled ? 'ON' : 'OFF'}</span>
          </label>
        </div>
        {chorusEnabled && (
          <div className="effect-controls">
            <div className="control-group">
              <label htmlFor="chorus-rate">
                Rate: {chorusRate.toFixed(1)} Hz
              </label>
              <input
                type="range"
                id="chorus-rate"
                min="0.1"
                max="10"
                step="0.1"
                value={chorusRate}
                onChange={(e) => setChorusRate(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="chorus-depth">
                Depth: {(chorusDepth * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="chorus-depth"
                min="0"
                max="1"
                step="0.01"
                value={chorusDepth}
                onChange={(e) => setChorusDepth(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="chorus-mix">
                Mix: {(chorusMix * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="chorus-mix"
                min="0"
                max="1"
                step="0.01"
                value={chorusMix}
                onChange={(e) => setChorusMix(parseFloat(e.target.value))}
              />
            </div>
          </div>
        )}
      </div>

      {/* Distortion Section */}
      <div className="effect-section">
        <div className="effect-header">
          <h3>Distortion</h3>
          <label className="effect-toggle">
            <input
              type="checkbox"
              checked={distortionEnabled}
              onChange={(e) => setDistortionEnabled(e.target.checked)}
            />
            <span className="toggle-text">
              {distortionEnabled ? 'ON' : 'OFF'}
            </span>
          </label>
        </div>
        {distortionEnabled && (
          <div className="effect-controls">
            <div className="control-group">
              <label htmlFor="distortion-drive">
                Drive: {(distortionDrive * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="distortion-drive"
                min="0"
                max="1"
                step="0.01"
                value={distortionDrive}
                onChange={(e) => setDistortionDrive(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="distortion-tone">
                Tone: {(distortionTone * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="distortion-tone"
                min="0"
                max="1"
                step="0.01"
                value={distortionTone}
                onChange={(e) => setDistortionTone(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="distortion-level">
                Level: {(distortionLevel * 100).toFixed(0)}%
              </label>
              <input
                type="range"
                id="distortion-level"
                min="0"
                max="1"
                step="0.01"
                value={distortionLevel}
                onChange={(e) => setDistortionLevel(parseFloat(e.target.value))}
              />
            </div>
          </div>
        )}
      </div>

      {/* Compressor Section */}
      <div className="effect-section">
        <div className="effect-header">
          <h3>Compressor</h3>
          <label className="effect-toggle">
            <input
              type="checkbox"
              checked={compressorEnabled}
              onChange={(e) => setCompressorEnabled(e.target.checked)}
            />
            <span className="toggle-text">
              {compressorEnabled ? 'ON' : 'OFF'}
            </span>
          </label>
        </div>
        {compressorEnabled && (
          <div className="effect-controls">
            <div className="control-group">
              <label htmlFor="comp-threshold">
                Threshold: {compressorThreshold.toFixed(0)} dB
              </label>
              <input
                type="range"
                id="comp-threshold"
                min="-60"
                max="0"
                step="1"
                value={compressorThreshold}
                onChange={(e) =>
                  setCompressorThreshold(parseFloat(e.target.value))
                }
              />
            </div>
            <div className="control-group">
              <label htmlFor="comp-ratio">
                Ratio: {compressorRatio.toFixed(1)}:1
              </label>
              <input
                type="range"
                id="comp-ratio"
                min="1"
                max="20"
                step="0.1"
                value={compressorRatio}
                onChange={(e) => setCompressorRatio(parseFloat(e.target.value))}
              />
            </div>
            <div className="control-group">
              <label htmlFor="comp-attack">
                Attack: {(compressorAttack * 1000).toFixed(1)}ms
              </label>
              <input
                type="range"
                id="comp-attack"
                min="0.001"
                max="0.1"
                step="0.001"
                value={compressorAttack}
                onChange={(e) =>
                  setCompressorAttack(parseFloat(e.target.value))
                }
              />
            </div>
            <div className="control-group">
              <label htmlFor="comp-release">
                Release: {(compressorRelease * 1000).toFixed(0)}ms
              </label>
              <input
                type="range"
                id="comp-release"
                min="0.01"
                max="1"
                step="0.01"
                value={compressorRelease}
                onChange={(e) =>
                  setCompressorRelease(parseFloat(e.target.value))
                }
              />
            </div>
          </div>
        )}
      </div>
    </div>
  );
};

export default EffectsTab;
