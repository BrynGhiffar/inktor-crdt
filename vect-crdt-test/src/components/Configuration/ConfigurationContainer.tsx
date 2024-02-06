import { DetailedHTMLProps, FC, HTMLAttributes } from "react";
import { twMerge } from "tailwind-merge";


type ConfigurationContainerProps = DetailedHTMLProps<HTMLAttributes<HTMLDivElement>, HTMLDivElement>;


export const ConfigurationContainer: FC<ConfigurationContainerProps> = ({ onClick, className, ...props}) => {
    return (
        <div
            className={twMerge("bg-[#181818] p-2 rounded-md border border-gray-600", className)}
            onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();
                if (onClick) onClick(e);
            }}
            {...props}
        ></div>
    )
};