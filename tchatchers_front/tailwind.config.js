/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./**/*"],
  theme: {
	fontFamily : {
		'tahoma': ['Tahoma', 'ui-serif', 'sans-serif']
	},
    extend: {
		animation: {
			'fade-out': 'fadeOut 1S ease-in-out',
			'fade-out-slow': 'fadeOut 5.5s ease-in-out',
			'fade-in': 'fadeIn 0.5s ease-in-out',
			'shake-2': 'shake 2s' 
		},
		keyframes: {
			fadeOut: {
				'0%':  { opacity: 1 },
				'100%': { opacity: 0 }
			},
			fadeIn : {
				'0%':  { opacity: 0 },
				'100%': { opacity: 1 }
			}
		},
		gridTemplateRows: {
			'12': 'repeat(12, minmax(0, 1fr))',
		},
		gridRow: {
				'span-10': 'span 10 / span 10',
				'span-11': 'span 11 / span 11',
				'span-4': 'span 4 / span 4',
			}
		}
  },
  plugins: [],
}
