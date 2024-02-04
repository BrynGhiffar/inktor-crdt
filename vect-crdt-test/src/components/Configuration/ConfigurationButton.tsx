import { DetailedHTMLProps, FC, HTMLAttributes } from "react";
import { twMerge } from "tailwind-merge";

type ConfigurationButtonProps = 
& DetailedHTMLProps<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement>;

export const ConfigurationButton: FC<ConfigurationButtonProps> = ({ children, className, ...props }) => {
    return (
        <button
            className={twMerge(
                "text-white bg-[#474747] px-2 py-1 rounded-md hover:bg-[#303030] transition ease-in-out",
                className
            )}
            { ...props }
        >{children}</button>
    );
};