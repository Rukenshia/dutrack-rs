document.addEventListener('DOMContentLoaded', () => {
  const menu = document.getElementById('nav-menu');
  document.getElementById('nav-toggle').addEventListener('click', () => {
    menu.classList.toggle('is-active');
  });
});