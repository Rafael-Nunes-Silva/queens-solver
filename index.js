import init, { get_images, get_gif } from "./pkg/queens_solver.js";



const errorMessageDiv = document.getElementById("error-message");
var current_input_url, current_output_img_url, current_output_gif_url;

function update_input_image(image) {
    if (current_input_url) {
        URL.revokeObjectURL(current_input_url);
    }

    current_input_url = URL.createObjectURL(image);
    const imagePreview = document.getElementById("input-img");
    imagePreview.src = current_input_url;
}

function update_output_image(image) {
    if (current_output_img_url) {
        URL.revokeObjectURL(current_output_img_url);
    }

    current_output_img_url = URL.createObjectURL(image);
    const imagePreview = document.getElementById("output-img");
    imagePreview.src = current_output_img_url;
}

function update_output_gif(gif) {
    if (current_output_gif_url) {
        URL.revokeObjectURL(current_output_gif_url);
    }

    current_output_gif_url = URL.createObjectURL(gif);
    const gifPreview = document.getElementById("output-gif");
    gifPreview.src = current_output_gif_url;
}

function showError(message) {
    errorMessageDiv.textContent = message;
    errorMessageDiv.style.display = "block";
}

init().then(function () {
    console.log("WASM loaded!");

    document.getElementById("image-input").addEventListener("change", async function (event) {
        errorMessageDiv.style.display = "none";
        const file = event.target.files[0];

        if (file) {
            update_input_image(file);

            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            try {
                const images_bytes = get_images(uint8Array);
                update_output_image(new Blob([images_bytes[images_bytes.length - 1]]));

                const gif_bytes = get_gif(uint8Array);
                update_output_gif(new Blob([gif_bytes]));
            } catch (error) {
                showError("The input image must be an empty Queens game");
            }

            event.target.value = null;
        }
    });
});
