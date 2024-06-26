document.addEventListener('DOMContentLoaded', () => {
    // Add any JavaScript animations or interactivity here
    const links = document.querySelectorAll('nav ul li a');
    links.forEach(link => {
        link.addEventListener('mouseover', () => {
            link.style.color = '#45a29e';
        });
        link.addEventListener('mouseout', () => {
            link.style.color = '';
        });
    });
});
