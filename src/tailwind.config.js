/** @type {import('tailwindcss').Config} */
module.exports = {
    content: ["src/*.rs", "index.html"],
    theme: {
      extend: {
        colors: {
          gold: {
            100: "#C5AC6A"
          }
        }
      },
    },
    plugins: [],
  }