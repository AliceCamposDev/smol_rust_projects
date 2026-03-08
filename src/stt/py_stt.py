import sys
import json
# print("instalando wsp")
import whisper
# print("instalado")

def main():
    # print("main")
    if len(sys.argv) < 2:
        print(json.dumps({"error": "No audio file provided"}))
        sys.exit(1)

    audio_path = sys.argv[1]
    try:
        model = whisper.load_model("medium") 
        result = model.transcribe(audio_path)
        print(json.dumps({"text": result["text"]}))
    except Exception as e:
        print(json.dumps({"error": str(e)}))
        sys.exit(1)

if __name__ == "__main__":
    main()