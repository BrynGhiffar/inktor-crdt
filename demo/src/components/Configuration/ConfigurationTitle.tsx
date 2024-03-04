import { FC } from "react"
import { twMerge } from "tailwind-merge";

type ConfigurationTitleProps = {
    title?: string,
    hideDeleteButton?: boolean,
    onClickDeleteButton?: () => void
};

export const ConfigurationTitle: FC<ConfigurationTitleProps> = ({ title, hideDeleteButton, onClickDeleteButton}) => {
    return (
        <div className={twMerge("grid gap-2", !hideDeleteButton && "grid-cols-[28px_auto]")}>
            {!hideDeleteButton && (
                <button
                    className="flex justify-center items-center rounded-md bg-red-400 aspect-square hover:bg-red-500 transition ease-in-out duration-100" 
                    onClick={onClickDeleteButton}
                >
                    <img src="/trash.svg" className="h-6 w-6"/>
                </button>
            )}
            <div className="text-white text-lg">{title}</div>
        </div>
    );
}