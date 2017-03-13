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
  const appInfo = document.getElementById('app-info');
  const card = document.getElementById('app-card');
  const doneInstalling = document.getElementById('done-installing');
  
  doneInstalling.addEventListener('click', () => {
    const renderedHeight = appInfo.clientHeight * 2;
    card.style.maxHeight = `${renderedHeight}px`;
    card.parentNode.querySelector('.card-footer').style.maxHeight = '0px';

    setTimeout(() => {
      document.getElementById('fence-setup-card').classList.toggle('hide', false);
    }, 100);
  });
});