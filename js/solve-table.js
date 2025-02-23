'use strict';

const url = new URL(window.location.href);
function updateParam(e) {
    const param_name = e.target.dataset.filter;
    let new_value = e.target.dataset.filterValue;
    if (new_value === undefined) {
        url.searchParams.delete(param_name);
    } else {
        url.searchParams.set(param_name, new_value);
    }
    sanitizeQueryParams()
    window.history.pushState(null, '', url.toString());
    handleFilterUpdate()
}

const isFmc = () => url.searchParams.get('event') == 'fmc'
const isSpeed = () => !isFmc()

function sanitizeQueryParams() {
    if (isSpeed()) {
        url.searchParams.delete('computer');
    }
    if (isFmc()) {
        url.searchParams.delete('blind');
        url.searchParams.delete('filters');
        url.searchParams.delete('macros');
    }
}

let xhr;

async function handleFilterUpdate() {
    // Update buttons disabled state
    const isFmc = url.searchParams.get('event') == 'fmc';
    const isSpeed = !isFmc;
    for (let element of document.querySelectorAll('.speed-only')) {
        if (isSpeed) {
            element.disabled = false;
        } else {
            element.disabled = true;
        }
    }
    for (let element of document.querySelectorAll('.fmc-only')) {
        if (isFmc) {
            element.disabled = false;
        } else {
            element.disabled = true;
        }
    }

    // Update buttons selected state
    let filterButtons = document.querySelectorAll('button.filter');
    for (let btn of filterButtons) {
        btn.classList.remove('selected', 'deselected');
        if (btn.dataset.filterValue == url.searchParams.get(btn.dataset.filter)) {
            btn.classList.add('selected');
        } else {
            btn.classList.add('deselected');
        }
    }

    // Load solves
    const solveTable = document.getElementById('solve-table');
    if (xhr) {
        xhr.abort();
    }
    xhr = new XMLHttpRequest();
    xhr.addEventListener('loadstart', () => {
        solveTable.innerHTML = "<span aria-busy=true>Loading solves…</span>";
    });
    xhr.addEventListener('load', () => {
        solveTable.replaceChildren(...xhr.responseXML.children);
    });
    xhr.addEventListener('error', () => {
        solveTable.innerHTML = "<p>Error loading solves</p>";
    });
    console.log(url.searchParams)
    xhr.open('GET', 'solves-table/all?' + url.searchParams);
    xhr.responseType = 'document';
    xhr.send();
}

document.addEventListener('click', (event) => {
    if (event.target.matches('button.filter')) {
        updateParam(event);
    }
});

window.addEventListener('load', handleFilterUpdate)
