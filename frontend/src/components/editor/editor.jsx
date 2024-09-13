import React, { useState } from 'react';
import EditableWindow from '../editable_window/editable_window';
import ReadableWindow from '../readable_window/readable_window';

const CodeEditorLayout = () => {
  const [codeData, setCodeData] = useState(null);
  const [dropDownOpt_1, setDropDownOpt_1] = useState('g++');
  const [dropDownOpt_2, setDropDownOpt_2] = useState('clang++');

  const handleCodeChange = (base64Code, functions, key) => {
    setCodeData({ base64Code, functions, key});
  };

  const handleDropDownChange = (option, windowNumber) => {
    if (windowNumber === 1) {
      setDropDownOpt_1(option);
    } else if (windowNumber === 2) {
      setDropDownOpt_2(option);
    }
  };

  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-4 p-4 h-screen bg-dark-background text-white">
      <EditableWindow onCodeChange={handleCodeChange} />
      <ReadableWindow windowNumber={1} selectedOption={dropDownOpt_1} onDropDownChange={handleDropDownChange} codeData={codeData} />
      <ReadableWindow windowNumber={2} selectedOption={dropDownOpt_2} onDropDownChange={handleDropDownChange} codeData={codeData} />
    </div>
  );
};

export default CodeEditorLayout;