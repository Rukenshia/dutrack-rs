document.addEventListener('DOMContentLoaded', () => {
  const android = document.getElementById('android');
  const iphone = document.getElementById('iphone');

  const guide = document.getElementById('guide');
  const qphone = document.getElementById('q-phone');

  [android, iphone].forEach(e => {
    e.addEventListener('click', () => {
      qphone.classList.toggle('hide', true);
      guide.classList.toggle('hide', false);
    })
  });
});