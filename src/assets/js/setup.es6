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

  // get app-info height
  const appInfo = document.getElementById('app-card');
  const card = document.getElementById('app-card-outer');
  const doneInstalling = document.getElementById('done-installing');
  
  doneInstalling.addEventListener('click', () => {
    var renderedHeight = appInfo.clientHeight;
    card.style.maxHeight = `6rem`;
    card.parentNode.querySelector('.card-footer').style.maxHeight = '0px';

    setTimeout(() => {
      document.getElementById('fence-setup-card').classList.toggle('hide', false);
    }, 100);
  });
});