document.addEventListener('DOMContentLoaded', function() {
  const menu = document.getElementById('nav-menu');
  document.getElementById('nav-toggle').addEventListener('click', function() {
    menu.classList.toggle('is-active');
  });
});