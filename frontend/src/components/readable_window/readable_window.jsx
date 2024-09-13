import React, { useEffect, useState, useCallback } from 'react';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { atomDark } from 'react-syntax-highlighter/dist/esm/styles/prism';
import { compileCode } from '../../service/compile_code_service';
import CryptoJS from 'crypto-js';
import './readable_window.css';

const ReadableWindow = ({ windowNumber, selectedOption, onDropDownChange, onArgumentsChange, codeData }) => {
  const [selectedCompilerData, setSelectedCompilerData] = useState('');
  const [typingTimeout, setTypingTimeout] = useState(null);
  const [args, setArgs] = useState([])
  const [argsString, setArgsString] = useState("")

  const debouncedCompile = useCallback((codeData, selectedOption, args) => {
    if (typingTimeout) {
      clearTimeout(typingTimeout);
    }

    const newTimeout = setTimeout(async () => {
      if (codeData) {
        try {
          const trimmedKey = codeData.key.replace(/\s+/g, '');
          const dataToHash = trimmedKey + selectedOption + (args === undefined ? "" : args.join(""));

          const sha256 = CryptoJS.SHA256(dataToHash).toString(CryptoJS.enc.Hex);
          
          var code_hash_var_name = "code_hash_" + selectedOption
          if (localStorage.getItem(code_hash_var_name) !== sha256 || localStorage.getItem(code_hash_var_name) === null) {

            console.log(localStorage.getItem(code_hash_var_name));
            console.log(sha256);

            localStorage.setItem(code_hash_var_name, sha256);
            const response = await compileCode(codeData.base64Code, codeData.functions, selectedOption, args, sha256);
            setSelectedCompilerData(response['output'][selectedOption].join("\n"));
          }
        } catch (error) {
          console.error('Error fetching compiler data:', error);
          setSelectedCompilerData('Error compiling code');
        }
      }
    }, 1000);

    setTypingTimeout(newTimeout);
  }, []);

  useEffect(() => {
    debouncedCompile(codeData, selectedOption, args);

    return () => {
      if (typingTimeout) {
        clearTimeout(typingTimeout);
      }
    };
  }, [codeData, selectedOption, args, debouncedCompile]);

  const handleArgumentsChange = (e) => {
    setArgsString(e.target.value);
    const argsArray = e.target.value.split(' ').filter(arg => arg.trim() !== '');
    setArgs(argsArray);
  };

  return (
    <div className="bg-gray-800 rounded-lg shadow-md overflow-auto scrollbar">
      <div className="p-4">
        <div className="flex items-center justify-between mb-4">
          <select
            className="flex h-10 rounded-md border border-gray-700 bg-gray-800 text-white px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50 mr-2"
            value={selectedOption}
            onChange={(e) => onDropDownChange(e.target.value, windowNumber)}
          >
            <option value="g++">g++</option>
            <option value="clang++">clang++</option>
          </select>
          <textarea
            className="flex-grow h-10 rounded-md border border-gray-700 bg-gray-800 text-white px-3 py-2 text-sm ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2"
            placeholder="Enter arguments here"
            value={argsString}
            onChange={handleArgumentsChange}

          />
        </div>
        <SyntaxHighlighter className="scrollbar" language="nasm" style={atomDark} wrapLines={true}>
          {selectedCompilerData}
        </SyntaxHighlighter>
      </div>
    </div>
  );
};

export default ReadableWindow;
