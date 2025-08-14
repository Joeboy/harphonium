import React from 'react';
import './KeyboardTab.css';

interface KeyboardTabProps {
  octaves: number;
  onOctavesChange: (octaves: number) => void;
}

const KeyboardTab: React.FC<KeyboardTabProps> = ({
  octaves,
  onOctavesChange,
}) => {
  const handleOctavesChange = (e: React.ChangeEvent<HTMLSelectElement>) => {
    onOctavesChange(parseFloat(e.target.value));
  };

  return (
    <div className="tab-content">
      <div className="settings-section">
        <div className="setting-item">
          <label>Octaves</label>
          <select value={octaves} onChange={handleOctavesChange}>
            <option value="1">1 Octave</option>
            <option value="1.5">1.5 Octaves</option>
            <option value="2">2 Octaves</option>
            <option value="2.5">2.5 Octaves</option>
            <option value="3">3 Octaves</option>
          </select>
        </div>
        <div className="setting-item">
          <label>
            <input type="checkbox" defaultChecked />
            Show Note Names
          </label>
        </div>
      </div>
    </div>
  );
};

export default KeyboardTab;
