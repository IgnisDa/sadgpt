/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        'spt': {
          'bg': '#343541'
        }
      }
    },
  },
  plugins: [],
}
