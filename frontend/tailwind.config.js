/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{js,jsx,ts,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        'dark-background': '#121212',
        'gray-800': '#333333',
        'gray-700': '#444444',
        'gray-500': '#888888',
      },
    },
  },
  plugins: [],
}