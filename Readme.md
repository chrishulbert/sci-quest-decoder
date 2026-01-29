# SCI Quest Decoder

![Aluminum Mallard](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn44.f3.vi581.li1.ci0.static.png)

This decodes and extracts the graphics from [classic Sierra SCI adventure games](https://sciwiki.sierrahelp.com/index.php/Sierra_SCI_Release_List#SCI0_.28late.29) such as Space Quest 3 and Police Quest 2.

![Plaid](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn60.f1.vi184.li7.animation.png)

To use this, run: `make run` - this will run against the assets of the built-in SCI fangame "New Years Mystery".

![Larry](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn716.f1.vi217.li0.animation.png)
![Patti](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn816.f1.vi219.li0.animation.png)

If you have Space Quest 3 or Police Quest 2, put them in `data/sq3` or `data/pq2` then run `make run-sq3` or `make run-pq2`.

![Robot](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn5.f1.vi88.li1.animation.png)

This supports SCI0 games (EGA), not SCI1 (VGA).

![Alien tourist](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn66.f2.vi278.li4.animation.png)

This generates big uncompressed PNGs. The reason they're so big is that I wanted the aspect ratio to be pixel-perfect, necessitating such large scale. To compress them, install `pngquant` and `apngasm` then run `make compress` to make them all a reasonable size.

![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn128.f3.vi520.li1.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn92.f1.vi222.li2.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn128.f3.vi520.li0.animation.png)
![Jello](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn92.f1.vi222.li5.animation.png)

By default this upscales using the XBRZ scaler. If you prefer the pixely look (honestly, I can never decide) then open renderer.rs and change `const USE_XBRZ: bool = false;`

## New Years Mystery

![Mystery](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn160.f1.vi50.li1.animation.png)

Includes [Doan Sephim's New Years Mystery](https://sciprogramming.com/fangames.php?action=review&id=3) SCI fangame so you can run this without needing a game. Thankyou so much for allowing this, Ryan!

## Others

![Scuba](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn12.f2.vi353.li0.animation.png)

If you're interested, I also wrote decoders for the following:

![Root Monster](https://github.com/chrishulbert/agi-quest-decoder/raw/main/readme/RootMonster.png)

* Sierra AGI: https://github.com/chrishulbert/agi-quest-decoder

![Digging](https://github.com/chrishulbert/digger-decoder/raw/main/readme/digging.png)

* Lemmings: https://github.com/chrishulbert/digger-decoder

![Dopefish](https://github.com/chrishulbert/dopefish-decoder/raw/main/Dopefish.png)

* Commander Keen: https://github.com/chrishulbert/dopefish-decoder

## References

![Rotating wireframe](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn42.f1.vi126.li1.animation.png)

* https://wiki.scummvm.org/index.php?title=SCI/Specifications
* https://github.com/scummvm/scummvm/tree/master/engines/sci
* https://slattstudio.com
* https://github.com/wjp/freesci-archive
* https://sciwiki.sierrahelp.com/index.php/SCI_Specifications
* https://www.agidev.com/articles/agispec/agispecs-7.html
* https://github.com/icefallgames/SCICompanion

## Art

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn1.f1.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn25.f2.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn47.f4.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn48.f1.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn525.f3.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn65.f2.static.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.picture.rn95.f1.static.png)

## Space Quest 3

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.intro.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.g.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.h.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.i.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.j.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.k.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.l.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.m.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.n.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.o.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.p.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.q.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.r.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.s.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.a.junk.t.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.b.escape.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.b.escape.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.g.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.h.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.i.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.j.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.k.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.l.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.m.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.n.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.o.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.p.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.q.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.r.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.s.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.t.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.c.phleebhut.u.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.d.monolith.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.g.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.h.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.i.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.j.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.l.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.m.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.n.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.o.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.p.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.q.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.r.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.e.ortega.s.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.g.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.h.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.i.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.j.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.k.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.l.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.m.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.n.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.f.pestulon.o.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.a.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.b.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.c.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.d.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.e.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.f.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.g.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.h.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.i.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.j.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.k.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.l.png)

![Picture](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/sq3/sq3.g.outro.m.png)

![Goodbye](https://github.com/chrishulbert/sci-quest-decoder/raw/main/readme/Output.view.rn67.f2.vi279.li4.animation.png)
