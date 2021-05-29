const numLettersInput = document.querySelector("input");

numLettersInput.addEventListener("input", undateValue);

function updateValue(e) {
    console.log(e.target.value);
}