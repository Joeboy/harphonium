import React, { useState, useMemo, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import './EffectsTab.css';
import throttle from 'lodash.throttle';

interface EffectsTabProps {
  isActive: boolean;
}

const EffectsTab: React.FC<EffectsTabProps> = ({ isActive }) => {
  // Fetch delay state from backend when tab becomes active
  useEffect(() => {
    if (isActive) {
      // Fetch delay time
      invoke('get_delay_time')
        .then((value) => setDelayTime(Number(value)))
        .catch((e) => console.error('Failed to get delay time:', e));
      // Fetch delay wet level
      invoke('get_delay_mix')
        .then((value) => setDelayWetLevel(Number(value)))
        .catch((e) => console.error('Failed to get delay mix:', e));
      // Fetch delay feedback
      invoke('get_delay_feedback')
        .then((value) => setDelayFeedback(Number(value)))
        .catch((e) => console.error('Failed to get delay feedback:', e));
    }
    // Optionally, do nothing when inactive
  }, [isActive]);

  // Delay controls
  const [delayTime, setDelayTime] = useState(0.25);
  const [delayWetLevel, setDelayWetLevel] = useState(0.3);
  const [delayFeedback, setDelayFeedback] = useState(0.4);

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
  const throttledSetDelayFeedback = useMemo(
    () =>
      throttle(async (value: number) => {
        try {
          await invoke('set_delay_feedback', { delayFeedback: value });
        } catch (error) {
          console.error('Failed to set delay feedback:', error);
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

  const handleDelayFeedbackChange = (value: number) => {
    setDelayFeedback(value);
    throttledSetDelayFeedback(value);
  };

  return (
    <div className="effects-tab">
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
            <label htmlFor="delay-feedback">
              Feedback: {(delayFeedback * 100).toFixed(0)}%
            </label>
            <input
              type="range"
              id="delay-feedback"
              min="0"
              max="0.95"
              step="0.01"
              value={delayFeedback}
              onChange={(e) =>
                handleDelayFeedbackChange(parseFloat(e.target.value))
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
    </div>
  );
};

export default EffectsTab;
