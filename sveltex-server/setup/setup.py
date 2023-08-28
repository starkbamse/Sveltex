import os
import importlib
import json
async def setup():
    #Reading core setup files
    core=os.listdir("setup/core")
    for x in range(len(core)):
        #Dynamically importing all the core setup files.
        mod = importlib.import_module("setup.core."+core[x]+".main")
        await mod.main() #All setup files have their starting point as a main function.
    extra=os.listdir("setup/extra")        
    for x in range(len(extra)):
        #Dynamically importing all the plugin setup files.
        mod = importlib.import_module("setup.extra."+extra[x]+".main")
        await mod.main()
    f=open("config/config.json")
    config=json.load(f)
    config["configured"]=True
    config=json.dumps(config)
    with open("config/config.json","w") as file:
        file.write(config)
    #Reading plugin setup files
    

