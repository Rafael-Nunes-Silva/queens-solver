import init, { run } from "./pkg/queens_solver.js";



var current_input_url, current_output_url;

function update_input_image(image) {
    if (current_input_url) {
        URL.revokeObjectURL(current_input_url);
    }

    current_input_url = URL.createObjectURL(image);
    const imagePreview = document.getElementById("input-img");
    imagePreview.src = current_input_url;
}

function update_output_image(image) {
    if (current_output_url) {
        URL.revokeObjectURL(current_output_url);
    }

    current_output_url = URL.createObjectURL(image);
    const imagePreview = document.getElementById("output-img");
    imagePreview.src = current_output_url;
}

init().then(function () {
    console.log("WASM loaded!");

    document.getElementById("image-input").addEventListener("change", async function (event) {
        const file = event.target.files[0];

        if (file) {
            update_input_image(file);

            const arrayBuffer = await file.arrayBuffer();
            const uint8Array = new Uint8Array(arrayBuffer);

            const images_bytes = run(uint8Array);
            for (let i = 0; i < images_bytes.length; i++) {
                const image = images_bytes[i];
                const blob = new Blob([image]);
                const url = URL.createObjectURL(blob);

                const newImage = document.createElement("img");
                newImage.setAttribute("src", url);
                document.getElementById("input-output-div").appendChild(newImage);
            }

            // const gif_bytes = run(uint8Array);
            // const blob = new Blob([gif_bytes], { type: file.type });
            // update_output_image(blob);

            event.target.value = null;
        }
    });
});
