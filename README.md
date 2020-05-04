# Custom Fn



## Custom functon to transform an image

## Remote function

### Deploy app to heroku
## Deply to heroku

https://devcenter.heroku.com/articles/git  

- add heroku app as git remote
```bash
heroku git:remote -a  secure-caverns-90265
```
 - set up **Procfile** for heroku app start
```bash
web: node app.js
```

- when you change app code push to heroku
```bash
git push heroku master
```


```
curl --location --request POST 'https://secure-caverns-90265.herokuapp.com/api/file' \
--header 'Content-Type: multipart/form-data; boundary=--------------------------279813939285989578662257' \
--form 'name=@/Users/rebeccapeltz/training-images/cloudinary-training.jpg'
```
