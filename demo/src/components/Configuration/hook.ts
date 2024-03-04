import { useState } from "react"

export const useShowHideColorPicker = () => {
    const [show, setShow] = useState(false);

    const showColorPicker = () => setShow(true);
    const hideColorPicker = () => setShow(false);
    const toggleColorPicker = () => setShow(prev => !prev);

    return { show, showColorPicker, hideColorPicker, toggleColorPicker };
}