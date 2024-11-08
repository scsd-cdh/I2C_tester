/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx,vue}",
  ],
  theme: {
    extend: {
      colors: {
        'light-background': '#f9f9f9',
        'light-foreground': '#333333',
        'dark-background': '#333333',
        'dark-foreground': '#f9f9f9',
        'primary': '#4F46E5',
        'secondary': '#3B82F6',
        'accent': '#F9A8D4',
      },
    },
  },
  plugins: [],
}

