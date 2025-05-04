async function submitForm() {
    const formData = new FormData(document.getElementById("wake_form"))

    if (formData.get("mac") === "ethan-pc") {
        formData.set("mac", "D8:43:AE:14:E7:FE")
    }

    if (formData.get("broadcast") === "") {
        formData.set("broadcast", "255.255.255.255");
    }

    if (formData.get("port") === "") {
        formData.set("port", "9");
    }

    try {
        const response = await fetch("/api/wake", {
            method: "post",
            body: formData
        });
        if (response.ok) {
            const status = document.getElementById("wake_status");
            status.innerText = `Magic packet sent to ${formData.get("mac")} on port ${formData.get("port")}`;
            status.style.fontStyle = "italic";
            status.style.color = "#00A66E";
        } else {
            const status = document.getElementById("wake_status");
            status.innerText = "Something went wrong! Check console for details.";
            status.style.fontStyle = "bold";
            status.style.color = "#9B2318";
        }
    } catch (e) {
        console.error(e);
    }
}