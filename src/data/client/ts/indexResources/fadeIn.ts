export default class FadeIn {

    public static runFadeIn(options: {
        initalDelay: number;
        spacing: number;
    }) {

        // set default options

        const elements = document.querySelectorAll("#fade-in");

        // order by `data-fade-order` attribute
        const orderedElements: {
            element: Element;
            order: number;
        }[] = Array.from(elements).map((element) => {
            return {
                element,
                order: parseInt(element.getAttribute("data-fade-order") || "0"),
            }
        })

        // fade in
        orderedElements.forEach((item) => {
                item.element.classList.add("hidden");
            setTimeout(() => {
                item.element.classList.add("fade-in");
                item.element.classList.remove("hidden");
            }, item.order * options.spacing + options.initalDelay);
        })

    }

}