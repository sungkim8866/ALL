# -*- coding: utf-8 -*-
import sys, io
from faster_whisper import WhisperModel

wav, out, model_name = sys.argv[1], sys.argv[2], sys.argv[3]
m = WhisperModel(model_name, device="cpu", compute_type="int8")
segs, info = m.transcribe(wav, language="ko", vad_filter=True, beam_size=5)

def ts(s):
    return "%d:%05.2f" % (int(s // 60), s % 60)

with io.open(out, "w", encoding="utf-8") as f:
    for s in segs:
        f.write("[%s - %s] %s\n" % (ts(s.start), ts(s.end), s.text.strip()))
print("transcript ->", out)
