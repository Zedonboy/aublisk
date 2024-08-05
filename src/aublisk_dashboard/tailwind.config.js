/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        "primary" : "black",
        "accent-dark" : "#254B3F",
        "accent": "#67BB7B"
      }
    },
  },
  plugins: [require('@tailwindcss/typography'), require("@tailwindcss/forms"), require('@tailwindcss/aspect-ratio')],
  prefix: "tw-"
}

