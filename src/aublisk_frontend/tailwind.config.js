/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{vue,js,ts,jsx,tsx}",
  ],
  important: true,
  theme: {
    extend: {
      colors: {
        "primary" : "black",
        "accent-dark" : "#254B3F",
        "accent": "#67BB7B"
      }
    },
  },
  plugins: [],
  prefix: "tw-",
  // corePlugins: {
  //   preflight: false, // This prevents Tailwind from resetting global styles
  // },
}

