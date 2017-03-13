document.addEventListener('DOMContentLoaded', () => {
  const android = document.getElementById('android');
  const iphone = document.getElementById('iphone');

  const guide_android = document.getElementById('guide-android');
  const guide_iphone = document.getElementById('guide-iphone');
  const qphone = document.getElementById('q-phone');

  android.addEventListener('click', () => {
    qphone.classList.toggle('hide', true);
    guide_android.classList.toggle('hide', false);
  });

  iphone.addEventListener('click', () => {
    qphone.classList.toggle('hide', true);
    guide_iphone.classList.toggle('hide', false);
  });
});