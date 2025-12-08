🧩 Générateur de Grilles ZigZag

Application Desktop Tauri (Rust + Vue)

Cette application permet de générer automatiquement des grilles de ZigZag (comme dans les magazines type TéléStar), avec sélection intelligente des mots selon leur longueur, leur fréquence dans la langue française et leur rareté naturelle (K, W, X, Z, Q, Y).
L’objectif : produire des grilles propres, lisibles, équilibrées et jouables, sans croisement de mots.

✨ Fonctionnalités

🖥 Application desktop multiplateforme grâce à Tauri

🎨 Interface moderne en Vue.js

🔤 Sélection intelligente des mots basée sur :

Probabilités par longueur (courts / moyens / longs)

Fréquences réelles des lettres en français

Gestion des lettres rares (K, W, X, Z, Q, Y)

Évitement des mots trop similaires

Prévention des grappes de lettres rares

🧮 Génération d’un chemin ZigZag unique, sans croisement

🧠 Choix automatique d’un mot mystère (non placé dans la grille)

📦 Export / futur : impression PDF, export JSON, sauvegarde de preset

🚀 Rust pour la logique + Vue pour l’UI = rapide, léger, efficace

📐 Logique de sélection des mots (résumé)

Catégories de longueur

Courts : 4–5 lettres

Moyens : 6–8 lettres

Longs : 9–12 lettres

Probabilités globales

Courts : 35 %

Moyens : 45 %

Longs : 20 %

Pondération naturelle par rareté
Chaque mot reçoit un score basé sur la fréquence naturelle des lettres françaises :

Ultra rares : K, W

Très rares : Z, Y

Rares : X

Peu fréquent : Q

Plus un mot contient de lettres rares, plus sa probabilité d’apparition diminue (jamais supprimée).

Sélection finale d’un mot

Tirage de la longueur

Pondération par rareté naturelle

Validation anti-doublons / anti-grappes

Ajout au chemin ZigZag

Sélection aléatoire du mot mystère (non posé)
