version=0.0121 

push:
	git add .
	git commit -m "rust api version ${version}"
	git tag ${version}
	git push origin ${version}
