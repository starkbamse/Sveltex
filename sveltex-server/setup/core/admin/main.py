import hashlib
import json
async def main():
    details_confirmed=False
    while not details_confirmed:
        username=input("Enter a new admin username: ")
        m=hashlib.sha256()
        password=input("Enter a new password: ")
        password=password.encode(encoding="UTF-8")
        m.update(password)
        password=m.hexdigest()
        decision=""
        while decision!="y" and decision!="n":
            decision=input(f'''
        Confirm your new details (y or n):
        USERNAME -> {username}
        PASSWORD_HASH -> {password}
        ''')
        if decision=="y":
            config=open("config/config.json")
            config=json.load(config)
            config["username"]=username
            config["password"]=password
            config=json.dumps(config)
            with open("config/config.json" ,"w") as file:
                file.write(config)
                details_confirmed=True
        
        