use coords::Vec3d;

pub struct Face {
    coords: [Vec3d; 3],
}

impl Face {
    pub fn new(a: Vec3d, b: Vec3d, c: Vec3d) -> Face {

    }
    pub fn intersect_dist(&self, ray:(Vec3d, Vec3d)) -> f64 {

    }
    pub fn normal(&self) -> Vector{
        
    }
}


//Intersection droite plan
//QVector<double> DEMManager::_lonIntersect(QVector<double> cartPos, QVector<double> cartDir, double lon){
//    //Intersection entre une ligne et un plan
//    //Computes the cartesian start point from the coordinates
//    double x = cartPos[0];
//    double y = cartPos[1];
//    double z = cartPos[2];
//    //Computes the direction in cartesian referential
//    double dx = cartDir[0];
//    double dy = cartDir[1];
//    double dz = cartDir[2];
//    //Formule demi-droite
//    //{x+k×dx; y+k×dy; z+k×dz}, k ∈ ℝ⁺
//
//    double lonRad = lon * DEGREE_TO_RADIAN;
//
//    //Équation du plan : x×sin(lon) - y×cos(lon) = 0
//    //Équation du point d’intersection : k tel que (x+k×dx)×sin(lon) - (y+k×dy)×cos(lon) = 0
//    // -> k = (y×cos(lon)-x×sin(lon))/(dx×sin(lon)-dy×cos(lon))
//    double k = (y*cos(lonRad)-x*sin(lonRad))/(dx*sin(lonRad)-dy*cos(lonRad));
//
//    //qDebug()<<"k"<<k;
//
//    //if (fabs(k) < DIST_EPSILON){
//    //    k = 0;
//    //}
//
//    if (isnan(k)){// || k < 0){
//        return QVector<double>{0, 0, 0};//Invalide, le plan est de l’autre côté
//    }
//
//    return QVector<double>{x+k*dx, y+k*dy, z+k*dz};
//}
