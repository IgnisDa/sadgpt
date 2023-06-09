/** @type {import('tailwindcss').Config} */
module.exports = {
  content: { 
    files: ["*.html", "./src/**/*.rs"],
  },
  theme: {
    extend: {
      colors: {
        'spt': {
          'creator': '#3b3c48',
          'bg': '#343541',
          'system': '#444654',
          'user': '#343541',
          'white': "#fbfbfb",
        }
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
  ]
}
