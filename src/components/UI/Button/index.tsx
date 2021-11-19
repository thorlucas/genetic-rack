import React, { ButtonHTMLAttributes } from "react";

type ButtonRole = 'primary' | 'secondary';

interface ButtonProps {
	role?: ButtonRole,
	type?: ButtonHTMLAttributes<HTMLButtonElement>['type'],
	children?: React.ReactNode,
}

const Button: React.FC<ButtonProps> = ({
	role = 'primary',
	type,
	children,
}) => {
	const color: string = (role === 'primary'
		? 'bg-blue-500 text-white text-opacity-90 font-semibold hover:bg-blue-600'
		: 'bg-gray-300 text-gray-900 hover:bg-gray-400'
	);
	
	return (
		<button
			className={ `px-8 py-4 rounded-md shadow-sm transition transition-color ${ color }` }
			type={ type }
		>
			<div className="flex items-center justify-center"> 
				{ children }
			</div>
		</button>
	);
};

export default Button;
