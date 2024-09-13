export const compileCode = async (base64Code, functions, compiler, args, sha256) => {

  const payload = {
    base64_code: base64Code,
    functions: functions,
    compiler: compiler,
    args: args === undefined || args === "" ? [""] : args,
    key: sha256
  };

  try {
    const API_URL = process.env.REACT_APP_API_URL;
    const response = await fetch(API_URL, {
      method: 'POST',
      body: JSON.stringify(payload)
    });
    return await response.json();
  } catch (error) {
    console.error('Error calling API:', error);
    throw error;
  }
};

export const extractFunctionNames = (codeValue) => {
  const functionRegex = /(?:^|\n|\r)\s*[\w<>_]+\s+([\w<>_]+)\s*\([^)]*\)\s*(?:const)?(?:\s*override)?(?:\s*=.*|;)?/g;
  let matches = [];
  let match;
  while ((match = functionRegex.exec(codeValue)) !== null) {
    matches.push(match[1]);
  }
  return matches;
};
