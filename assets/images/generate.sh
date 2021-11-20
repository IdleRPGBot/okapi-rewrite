#!/usr/bin/bash
mkdir -p out
cp -r raw/adventures out
cp raw/*.png out
cp -r raw/casts out

cd out/casts

for image in human.png elf.png dwarf.png orc.png jikill.png; do
    mogrify -resize 60 -quality 100% "${image}"
done

for image in thief.png paragon.png ranger.png warrior.png mage.png raider.png ritualist.png; do
    mogrify -resize 60 -quality 100% "${image}"
done

for image in leader.png officer.png member.png; do
    mogrify -resize 55 -quality 100% "${image}"
done

for image in sword.png shield.png knife.png dagger.png spear.png hammer.png axe.png bow.png howlet.png scythe.png wand.png; do
    mogrify -resize 80 -quality 100% "${image}"
done

for image in contributor.png designer.png developer.png gamedesigner.png gamemaster.png support.png tester.png veteran.png; do
    mogrify -resize 45 -quality 100% "${image}"
done

img-optimize ./ --jpg --png
