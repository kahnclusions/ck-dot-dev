/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}"],
  theme: {
    fontFamily: {
      mono: ["AnonymousPro"],
      serif: ["Volume Tc"],
      display: ["Silkscreen"],
      dos: ["PerfectDOS"],
      ospx: ["OpenSansPX"],
    },
    extend: {
      colors: {
        base: "hsl(var(--color-base) / <alpha-value>)",
        surface: "hsl(var(--color-surface) / <alpha-value>)",
        overlay: "hsl(var(--color-overlay) / <alpha-value>)",
        muted: "hsl(var(--color-muted) / <alpha-value>)",
        subtle: "hsl(var(--color-subtle) / <alpha-value>)",
        text: "hsl(var(--color-text) / <alpha-value>)",
        love: "hsl(var(--color-love) / <alpha-value>)",
        gold: "hsl(var(--color-gold) / <alpha-value>)",
        rose: "hsl(var(--color-rose) / <alpha-value>)",
        pine: "hsl(var(--color-pine) / <alpha-value>)",
        foam: "hsl(var(--color-foam) / <alpha-value>)",
        iris: "hsl(var(--color-iris) / <alpha-value>)",
        "hl-low": "hsl(var(--color-hl-low) / <alpha-value>)",
        "hl-med": "hsl(var(--color-hl-med) / <alpha-value>)",
        "hl-high": "hsl(var(--color-hl-high) / <alpha-value>)",
      },
    },
  },
  plugins: [],
};
