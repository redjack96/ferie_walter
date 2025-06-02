use std::cell::{Ref, RefCell, RefMut};
use std::rc::Rc;

/// Un alias per Rc<RefCell<T>>.
#[derive(Clone, Debug, Default)]
pub struct Common<T>(Rc<RefCell<T>>);


impl<T> Common<T> {
    pub fn new(t: T) -> Common<T> {
        Common(Rc::new(RefCell::new(t)))
    }
    /// Restituisce un riferimento immutabile. E' possibile leggere tutte le volte che si vuole in uno scope.
    /// Attenzione: se nello scope c'è get_mut() si rischia il PANIC anche con read(). Preferire write(T) a get_mut() quando possibile
    pub fn read(&self) -> Ref<'_, T> {
        self.0.borrow()
    }

    pub fn copy(&self) -> T where T: Copy {
        self.0.borrow().clone()
    }

    /// Restituisce un riferimento mutabile ed esclusivo e sostituisce il valore T con quello indicato.
    /// Attenzione: se nello scope c'è un borrow in lettura con read() e poi usi write() rischi il PANIC.
    /// Utilizzare solo quando il tipo T ha dimensione in byte limitata (<100B) ed è poco oneroso clonarlo.
    pub fn write(&self, val: T) {
        *self.0.borrow_mut() = val;
    }

    /// Apre un borrow mutabile ed esclusivo per scrivere in T. E' possibile scrivere solo una volta in uno scope.
    /// Utilizzare solo quando è necessario chiamare un metodo di T con receiver &mut self
    /// o quando si vuole cambiare solo un campo invece che tutto il T.
    ///
    /// Attenzione, ridurre il più possibile lo scope (cioè le parentesi graffe attorno al metodo),
    /// per rilasciare il più velocemente possibile il borrow e far procedere altre scritture o letture.
    /// In altre parole, preferisci il primo metodo al secondo nell'esempio che segue:
    ///
    /// ```
    ///
    /// use utils::common::Common;
    ///
    /// struct Foo {
    ///     ok: bool,
    ///     message: Common<Option<String>>,
    /// }
    ///
    /// let t = Foo{ok:true, message: Common::new(None)};
    ///
    /// // Primo metodo (da preferire)
    /// t.message.write(Some("Hello world".to_string()));
    /// // Secondo metodo (da evitare)
    /// if let Some(mut str) = t.message.get_mut() {
    ///     *str = Some("Hello world".to_string());
    ///     // Se scommenti la prossima riga causi un panic
    ///     // let deadlock = t.message.read().as_ref();
    /// } // Borrow mutabile rilasciato qui
    /// ```
    pub fn get_mut(&self) -> RefMut<'_, T> {
        self.0.borrow_mut()
    }

}

impl Common<u32> {
    pub fn sum(&self, num: u32) {
        *self.0.borrow_mut() += num;
    }

    pub fn sub(&self, num: u32) {
        *self.0.borrow_mut() -= num;
    }
}

pub mod serde_common {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use crate::common::Common;

    pub fn serialize<S, T>(value: &Common<T>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Serialize,
    {
        value.read().serialize(serializer)
    }

    pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Common<T>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Common::new(value))
    }
}