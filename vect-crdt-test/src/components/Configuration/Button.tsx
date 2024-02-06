import { DetailedHTMLProps, FC, HTMLAttributes } from "react";
import { twMerge } from "tailwind-merge";

type ColorButtonProps =
& DetailedHTMLProps<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement>
& {
    pickerColor: [number, number, number, number]
} 

export const ColorButton: FC<ColorButtonProps> = ({ onClick, children, pickerColor, ...props }) => {
    const [ red, green, blue, opacity ] = pickerColor;
    return (
    <div className="relative flex items-start">
        <button
            style={{ background: `rgba(${red}, ${green}, ${blue}, ${opacity})` }}
            className="w-10 h-3 cursor-pointer border border-black"
            onClick={(e) => { 
                e.stopPropagation();
                e.preventDefault(); 
                if (onClick) { onClick(e) }
            }}
            { ...props }
        />
        { children }
    </div>
    );
}

type TrashButtonProps = 
& DetailedHTMLProps<HTMLAttributes<HTMLButtonElement>, HTMLButtonElement>
& {
    add?: boolean
}

export const TrashButton: FC<TrashButtonProps> = ({ className, add, ...props }) => {
    return (
        <button
            className={twMerge(
                !add ? "bg-red-400 hover:bg-red-500" : "bg-blue-400 hover:bg-blue-500",
                "flex justify-center items-center rounded-md aspect-square transition ease-in-out duration-100", 
                className
            )}
            { ...props }
        >
            {!add ? (
                <img src="/trash.svg" alt="T" className="h-6 w-6"/>    
            ) : (
                <img src="/add.svg" alt="A" className="h-6 w-6"/>
            )}
        </button>
    )
}