import json
async def main():
    details_confirmed=False
    while not details_confirmed:
        print('''
    Now it is time to enter your mongodb connection string. 
    E.g. mongodb://myDBReader:D1fficultP%40ssw0rd@mongodb0.example.com:27017/?authSource=admin

        ''')
        mongo=input("Enter your full mongodb connection string: ")
        decision=""
        while decision!="y" and decision!="n":
            decision=input(f'''
        Confirm your mongodb connection string (y or n):
        -> {mongo}
        ''')
        if decision=="y":
            config=open("config/config.json")
            config=json.load(config)
            config["mongo"]=mongo
            config=json.dumps(config)
            with open("config/config.json" ,"w") as file:
                file.write(config)
                details_confirmed=True        